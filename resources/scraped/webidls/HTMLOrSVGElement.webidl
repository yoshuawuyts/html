interface mixin HTMLOrSVGElement {
  [SameObject] readonly attribute DOMStringMap dataset;
  attribute DOMString nonce; // intentionally no [CEReactions]

  [CEReactions] attribute boolean autofocus;
  [CEReactions] attribute long tabIndex;
  undefined focus(optional FocusOptions options = {});
  undefined blur();
};