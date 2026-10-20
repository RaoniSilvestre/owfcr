use std::sync::Arc;
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::mpsc::{self, Sender};
use std::thread;
use rusqlite::{Connection, params};
use tracing::{error, info, instrument};
use crate::types::Trial;

#[derive(Clone)]
pub struct Repo {
    sender: Sender<Trial>,
}

impl Repo {
    pub fn new(path: &str, executions: Arc<AtomicI64>) -> Self {
        let (sender, receiver) = mpsc::channel::<Trial>();
        let db_path = path.to_string();

        thread::spawn(move || {
            let mut conn = Connection::open(&db_path).expect("Failed to open DB");
            
            conn.execute_batch("
                PRAGMA journal_mode = WAL;
                PRAGMA synchronous = NORMAL;
            ").ok();

            conn.execute(
                "CREATE TABLE IF NOT EXISTS trials (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    algorithm TEXT NOT NULL,
                    instance TEXT NOT NULL,
                    instance_size INTEGER NOT NULL,
                    duration_ms INTEGER NOT NULL,
                    total_cost REAL NOT NULL,
                    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
                    seed INTEGER
                )",
                [],
            ).unwrap();

            conn.execute(
                "CREATE TABLE IF NOT EXISTS edges (
                    trial_id INTEGER NOT NULL,
                    u INTEGER NOT NULL,
                    v INTEGER NOT NULL,
                    flow INTEGER NOT NULL,
                    FOREIGN KEY(trial_id) REFERENCES trials(id)
                )",
                [],
            ).unwrap();

            while let Ok(trial) = receiver.recv() {
                if let Err(e) = Self::save_trial_internal(&mut conn, &trial) {
                    error!("Error while saving trial: {e}");
                }
                executions.fetch_add(1, Ordering::Relaxed);
            }
        });

        Self { sender }
    }

    pub fn save_trial(&self, trial: Trial) -> color_eyre::Result<()> {
        self.sender.send(trial)?;
        Ok(())
    }

    #[instrument(skip_all, name = "Repository")]
    fn save_trial_internal(conn: &mut Connection, trial: &Trial) -> color_eyre::Result<()> {
        let tx = conn.transaction()?;

        info!("Saving trial {}, {}.", trial.metadata.algorithm, trial.metadata.instance);

        tx.execute(
            "INSERT INTO trials (algorithm, instance, instance_size, duration_ms, total_cost, seed) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                trial.metadata.algorithm,
                trial.metadata.instance,
                i64::from(trial.metadata.instance_size),
                trial.metadata.duration.as_millis() as i64,
                trial.solution.cost,
                trial.metadata.seed.map(i64::from), 
            ],
        )?;

        let trial_id = tx.last_insert_rowid();

        {
            let mut stmt = tx.prepare_cached("INSERT INTO edges (trial_id, u, v, flow) VALUES (?1, ?2, ?3, ?4)")?;
            let n = trial.solution.parents.len();
            for i in 0..n {
                stmt.execute(params![
                    trial_id,
                    i as i64,
                    trial.solution.parents[i] as i64,
                    trial.solution.flows[i] 
                ])?;
            }
        }

        tx.commit()?;

        info!("{}, {} trial saved.", trial.metadata.algorithm, trial.metadata.instance);
        Ok(())
    }
}
