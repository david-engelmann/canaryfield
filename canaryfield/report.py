"""The catch table, rendered as plain text and as JSON."""

import json


def to_json(result):
    return json.dumps(result, indent=2, sort_keys=True) + "\n"


def to_text(result):
    gate_names = result["gates"]
    lines = []
    lines.append("canaryfield catch table")
    lines.append("=======================")
    lines.append("")
    lines.append(f"target           {result['target']}")
    lines.append(f"target unchanged {str(result['target_unchanged']).lower()}")
    for name in gate_names:
        lines.append(f"clean exit for gate {name} is {result['clean_exits'][name]}")
    lines.append("")
    header = ["seed", "name", "planted file"] + gate_names
    rows = []
    for row in result["rows"]:
        rows.append(
            [row["seed"], row["name"], row["planted_file"] or "none"]
            + [row["gates"][name] for name in gate_names]
        )
    widths = [
        max(len(header[index]), *(len(row[index]) for row in rows))
        for index in range(len(header))
    ]

    def render(cells):
        return "  ".join(
            cell.ljust(widths[index]) for index, cell in enumerate(cells)
        ).rstrip()

    lines.append(render(header))
    lines.append(render(["-" * width for width in widths]))
    for row in rows:
        lines.append(render(row))
    lines.append("")
    lines.append("summary per gate")
    for name in gate_names:
        counts = result["summary"][name]
        lines.append(
            f"  {name}  caught {counts['caught']}"
            f"  waved-through {counts['waved-through']}"
            f"  not-applicable {counts['not-applicable']}"
            f"  gate-broken {counts['gate-broken']}"
        )
    return "\n".join(lines) + "\n"
