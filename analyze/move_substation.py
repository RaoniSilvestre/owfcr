#!/usr/bin/env python3
"""Reposiciona a subestação de cada instância em um novo diretório.

Para cada arquivo `*.turb` do diretório de entrada:

1. identifica a linha da subestação (peso negativo) e as turbinas (peso positivo);
2. calcula a nova posição da subestação conforme o modo escolhido;
3. escreve `<stem>_<modo>.turb` no diretório de saída, mantendo o peso negativo
   original e as turbinas inalteradas;
4. copia o `<stem>.cable` correspondente.

Modos (`--mode`):
    middle  — centroide das turbinas.
    right   — à direita do campo: (max_x + margin * largura, y_centroide).
    top     — acima do campo:      (x_centroide, max_y + margin * altura).
    bottom  — abaixo do campo:     (x_centroide, min_y - margin * altura).

`--margin` é a fração da largura/altura do campo adicionada além da borda
(default: 0.25). Não é usada no modo `middle`.

Exemplos:
    python3 analyze/move_substation.py --mode middle
    python3 analyze/move_substation.py --mode right --margin 0.5 --output instances_pos
"""

from __future__ import annotations

import argparse
import shutil
from pathlib import Path

TURB_EXT = ".turb"
CABLE_EXT = ".cable"

# Modo -> sufixo do arquivo gerado.
MODE_SUFFIX = {
    "middle": "mid",
    "right": "right",
    "top": "top",
    "bottom": "bottom",
}


def parse_turb(path: Path) -> tuple[list[str], int] | None:
    """Lê o arquivo `.turb` e separa subestação e turbinas.

    Retorna `(linhas_das_turbinas, w_da_subestacao)` ou `None` se o arquivo não
    tiver exatamente uma subestação.
    """
    turbine_lines: list[str] = []
    substation_w: int | None = None

    with path.open(encoding="utf-8") as f:
        for raw in f:
            line = raw.rstrip("\n")
            if not line.strip():
                continue
            parts = line.split()
            x, y, w = float(parts[0]), float(parts[1]), int(parts[2])
            if w < 0:
                if substation_w is not None:
                    return None  # mais de uma subestação
                substation_w = w
            else:
                turbine_lines.append(line)

    if substation_w is None:
        return None

    return turbine_lines, substation_w


def turbine_stats(turbine_lines: list[str]) -> tuple[float, float, float, float, float, float] | None:
    """Calcula centroide e bounding box das turbinas.

    Retorna `(cx, cy, min_x, max_x, min_y, max_y)` ou `None` se não houver turbinas.
    """
    if not turbine_lines:
        return None

    xs = []
    ys = []
    for line in turbine_lines:
        parts = line.split()
        xs.append(float(parts[0]))
        ys.append(float(parts[1]))

    return (
        sum(xs) / len(xs),
        sum(ys) / len(ys),
        min(xs),
        max(xs),
        min(ys),
        max(ys),
    )


def new_position(mode: str, stats: tuple[float, float, float, float, float, float], margin: float) -> tuple[float, float]:
    cx, cy, min_x, max_x, min_y, max_y = stats

    match mode:
        case "middle":
            return cx, cy
        case "right":
            return max_x + margin * (max_x - min_x), cy
        case "top":
            return cx, max_y + margin * (max_y - min_y)
        case "bottom":
            return cx, min_y - margin * (max_y - min_y)
        case _:
            raise ValueError(f"modo desconhecido: {mode}")


def move_substations(input_dir: Path, output_dir: Path, mode: str, margin: float) -> None:
    output_dir.mkdir(parents=True, exist_ok=True)

    suffix = MODE_SUFFIX[mode]
    processed = 0
    skipped = 0

    for turb_path in sorted(input_dir.glob(f"*{TURB_EXT}")):
        parsed = parse_turb(turb_path)
        if parsed is None:
            print(f"[SKIP] {turb_path.name}: subestação ausente ou ambígua")
            skipped += 1
            continue

        turbine_lines, w = parsed
        stats = turbine_stats(turbine_lines)
        if stats is None:
            print(f"[SKIP] {turb_path.name}: nenhuma turbina encontrada")
            skipped += 1
            continue

        nx, ny = new_position(mode, stats, margin)

        with (output_dir / f"{turb_path.stem}_{suffix}{TURB_EXT}").open("w", encoding="utf-8") as f:
            f.write(f"{nx:.10g} {ny:.10g} {w}\n")
            for line in turbine_lines:
                f.write(f"{line}\n")

        cable_path = turb_path.with_suffix(CABLE_EXT)
        if cable_path.exists():
            shutil.copyfile(cable_path, output_dir / f"{turb_path.stem}_{suffix}{CABLE_EXT}")
        else:
            print(f"[WARN] {turb_path.name}: cabo ausente ({cable_path.name})")

        processed += 1

    print(f"Modo: {mode} | Margin: {margin} | Processados: {processed} | Pulados: {skipped} | Saída: {output_dir}")


def main() -> None:
    parser = argparse.ArgumentParser(
        description="Reposiciona a subestação de cada instância em um novo diretório."
    )
    parser.add_argument(
        "--input",
        default="../instances",
        help="Diretório com os arquivos .turb originais (default: ./instances)",
    )
    parser.add_argument(
        "--output",
        default="./instances_pos",
        help="Diretório de saída (default: ./instances_pos)",
    )
    parser.add_argument(
        "--mode",
        choices=sorted(MODE_SUFFIX),
        default="middle",
        help="Onde colocar a subestação: middle, right, top ou bottom (default: middle)",
    )
    parser.add_argument(
        "--margin",
        type=float,
        default=0.25,
        help="Fração da largura/altura do campo além da borda (default: 0.25)",
    )
    args = parser.parse_args()

    move_substations(Path(args.input), Path(args.output), args.mode, args.margin)


if __name__ == "__main__":
    main()
