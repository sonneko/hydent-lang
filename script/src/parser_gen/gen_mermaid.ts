import { Grammar } from "./ast";

function generateMermaid(grammar: Grammar): string {
    let lines: string[] = ["classDiagram"];

    for (const rule of grammar) {
        if (rule.kind === "Branch") {
            lines.push(`    class _${rule.name}`);
            for (const variant of rule.variants) {
                lines.push(`    ${rule.name} <|-- ${variant.name}`);
            }
        } else if (rule.kind === "Product") {
            lines.push(`    class ${rule.name} {`);
            for (const member of rule.members) {
                if (member.kind === "Field") {
                    const mod = member.type.modifier === 'List' ? '[]' :
                        member.type.modifier === 'Option' ? '?' : '';
                    lines.push(`        +${member.type.name}${mod} ${member.name}`);
                }
            }
            lines.push(`    }`);

            for (const member of rule.members) {
                if (member.kind === "Field") {
                    lines.push(`    ${rule.name} --> ${member.type.name}`);
                }
            }
        }
    }
    return lines.join("\n");
}

export function generateMermaidHtml(grammar: Grammar): string {
    return `<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>Grammar Visualizer</title>
    <script src="https://cdn.jsdelivr.net/npm/mermaid/dist/mermaid.min.js"></script>
    <script src="https://cdn.jsdelivr.net/npm/svg-pan-zoom@3.6.1/dist/svg-pan-zoom.min.js"></script>
    <style>
        body, html { 
            margin: 0; padding: 0; width: 100%; height: 100%; overflow: hidden; 
            background-color: #f8f9fa; font-family: sans-serif;
        }
        h1 { 
            position: absolute; top: 10px; left: 20px; z-index: 1000; 
            background: rgba(255, 255, 255, 0.8); padding: 5px 15px; 
            border-radius: 5px; box-shadow: 0 2px 5px rgba(0,0,0,0.1);
            pointer-events: none; /* 下の図を操作しやすくする */
        }
        /* Mermaidが生成する前のテキストを隠す */
        .mermaid { visibility: hidden; width: 100vw; height: 100vh; background: white; }
    </style>
</head>
<body>
    <h1>Grammar Structure Visualizer</h1>
    <div>
        <pre class="mermaid">
            ${generateMermaid(grammar)}
        </pre>
    </div>

    <script>
        mermaid.initialize({ 
            startOnLoad: false,
            theme: 'neutral',
            maxTextSize: 1000000,
            // ここが重要：SVGが勝手に縮小されないようにする
            class: { useMaxWidth: false } 
        });

        async function draw() {
            // 1. レンダリング
            await mermaid.run({
                nodes: document.querySelectorAll('.mermaid'),
            });

            // 2. 生成されたSVGを取得
            const svgElement = document.querySelector('.mermaid svg');
            if (!svgElement) return;

            // 3. SVGのスタイルをコンテナにフィットさせる
            svgElement.style.visibility = 'visible';
            svgElement.style.width = '100%';
            svgElement.style.height = '100%';
            svgElement.style.maxWidth = 'none'; // 上限を解除

            // 4. svg-pan-zoomを適用
            svgPanZoom(svgElement, {
                zoomEnabled: true,
                controlIconsEnabled: true,
                fit: true,
                center: true,
                minZoom: 0.01, // 巨大な図に対応するため小さめに設定
                maxZoom: 20
            });
        }

        draw();
    </script>
</body>
</html>`;
}
