[Exposed=Window]
interface HTMLBaseElement : HTMLElement {
  [HTMLConstructor] constructor();

  [CEReactions] attribute USVString href;
  [CEReactions] attribute DOMString target;
};