import { describe, expect, it } from "vitest";
import { escapeHtml, formatNoteContent } from "./note-format";

describe("formatNoteContent", () => {
  it("still renders the markdown it always did", () => {
    expect(formatNoteContent("# Title")).toBe("<h1>Title</h1>");
    expect(formatNoteContent("a **b** and *c*")).toBe(
      "a <strong>b</strong> and <em>c</em>",
    );
    expect(formatNoteContent("- one\n- two")).toContain("<ul>");
    expect(formatNoteContent("`x`")).toBe("<code>x</code>");
    expect(formatNoteContent("line1\nline2")).toBe("line1<br/>line2");
  });

  it("never lets a note inject markup or event handlers", () => {
    const attacks = [
      '<img src=x onerror="alert(1)">',
      "<script>alert(1)</script>",
      '<a href="javascript:alert(1)">x</a>',
      "<svg onload=alert(1)>",
      '"><iframe src=//evil></iframe>',
    ];
    for (const attack of attacks) {
      const html = formatNoteContent(attack);
      expect(html).not.toMatch(/<(img|script|a|svg|iframe)\b/i);
      expect(html).toContain("&lt;");
    }
  });

  it("escapes inside markdown too", () => {
    const html = formatNoteContent("**<img src=x onerror=1>**");
    expect(html).toBe("<strong>&lt;img src=x onerror=1&gt;</strong>");
    expect(formatNoteContent("```\n<b>raw</b>\n```")).toBe(
      "<pre><code>&lt;b&gt;raw&lt;/b&gt;<br/></code></pre>",
    );
  });

  it("shows ampersands and quotes as text", () => {
    expect(escapeHtml(`Tom & "Jerry" <3 's`)).toBe(
      "Tom &amp; &quot;Jerry&quot; &lt;3 &#39;s",
    );
  });
});
