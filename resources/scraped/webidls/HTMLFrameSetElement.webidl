[Exposed=Window]
interface HTMLFrameSetElement : HTMLElement {
  [HTMLConstructor] constructor();

  [CEReactions] attribute DOMString cols;
  [CEReactions] attribute DOMString rows;
};
HTMLFrameSetElement includes WindowEventHandlers;