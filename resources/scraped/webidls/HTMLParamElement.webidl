[Exposed=Window]
interface HTMLParamElement : HTMLElement {
  [HTMLConstructor] constructor();

  [CEReactions] attribute DOMString name;
  [CEReactions] attribute DOMString value;
  [CEReactions] attribute DOMString type;
  [CEReactions] attribute DOMString valueType;
};