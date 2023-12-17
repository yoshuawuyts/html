[Exposed=Window]
interface HTMLTemplateElement : HTMLElement {
  [HTMLConstructor] constructor();

  readonly attribute DocumentFragment content;
  [CEReactions] attribute DOMString shadowRootMode;
  [CEReactions] attribute boolean shadowRootDelegatesFocus;
};