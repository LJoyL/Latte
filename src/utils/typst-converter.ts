export class TypstConverter {
  static toHtml(typst: string): string {
    if (!typst) return '';

    // Escape HTML special chars first? TipTap might handle it, but let's be safe with basic escaping if we were manual.
    // Actually, we'll produce HTML string.
    
    const lines = typst.split('\n');
    let html = '';
    let inList = false;

    for (const line of lines) {
      const trimmed = line.trim();
      
      if (trimmed.startsWith('- ')) {
        if (!inList) {
          html += '<ul>';
          inList = true;
        }
        html += `<li>${this.parseInline(trimmed.substring(2))}</li>`;
      } else {
        if (inList) {
          html += '</ul>';
          inList = false;
        }
        
        if (trimmed.startsWith('= ')) {
          html += `<h1>${this.parseInline(trimmed.substring(2))}</h1>`;
        } else if (trimmed.startsWith('== ')) {
          html += `<h2>${this.parseInline(trimmed.substring(3))}</h2>`;
        } else if (trimmed.startsWith('=== ')) {
          html += `<h3>${this.parseInline(trimmed.substring(4))}</h3>`;
        } else if (trimmed === '') {
          // Empty line, maybe ignore or p? TipTap prefers <p></p>
        } else {
          html += `<p>${this.parseInline(trimmed)}</p>`;
        }
      }
    }
    
    if (inList) html += '</ul>';
    
    return html;
  }

  static parseInline(text: string): string {
    // Bold *text*
    text = text.replace(/\*(.*?)\*/g, '<strong>$1</strong>');
    // Italic _text_
    text = text.replace(/_(.*?)_/g, '<em>$1</em>');
    return text;
  }

  static fromHtml(html: string): string {
    // This is a naive implementation.
    // For a robust solution, we should traverse the TipTap JSON, but parsing HTML string is "okay" for MVP.
    // Actually, TipTap can give us JSON. Parsing JSON to Typst is cleaner.
    // But for now, let's assume we get HTML from editor.getHTML()
    
    const parser = new DOMParser();
    const doc = parser.parseFromString(html, 'text/html');
    return this.serializeNode(doc.body);
  }

  static serializeNode(node: Node): string {
    let text = '';
    
    for (const child of Array.from(node.childNodes)) {
      if (child.nodeType === Node.TEXT_NODE) {
        text += child.textContent;
      } else if (child.nodeType === Node.ELEMENT_NODE) {
        const el = child as HTMLElement;
        const tagName = el.tagName.toLowerCase();
        
        if (tagName === 'p') {
          text += this.serializeNode(el) + '\n\n';
        } else if (tagName === 'h1') {
          text += `= ${this.serializeNode(el)}\n\n`;
        } else if (tagName === 'h2') {
          text += `== ${this.serializeNode(el)}\n\n`;
        } else if (tagName === 'h3') {
          text += `=== ${this.serializeNode(el)}\n\n`;
        } else if (tagName === 'ul') {
          text += this.serializeNode(el);
        } else if (tagName === 'li') {
          text += `- ${this.serializeNode(el)}\n`;
        } else if (tagName === 'strong' || tagName === 'b') {
          text += `*${this.serializeNode(el)}*`;
        } else if (tagName === 'em' || tagName === 'i') {
          text += `_${this.serializeNode(el)}_`;
        } else {
          text += this.serializeNode(el);
        }
      }
    }
    
    return text;
  }
}
