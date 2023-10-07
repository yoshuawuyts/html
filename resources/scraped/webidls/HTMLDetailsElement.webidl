[Exposed=Window]
interface HTMLDetailsElement : HTMLElement {
  [HTMLConstructor] constructor();

  [CEReactions] attribute DOMString name;
  [CEReactions] attribute boolean open;
};