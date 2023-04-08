partial interface HTMLTableElement {
  [CEReactions] attribute DOMString align;
  [CEReactions] attribute DOMString border;
  [CEReactions] attribute DOMString frame;
  [CEReactions] attribute DOMString rules;
  [CEReactions] attribute DOMString summary;
  [CEReactions] attribute DOMString width;

  [CEReactions] attribute [LegacyNullToEmptyString] DOMString bgColor;
  [CEReactions] attribute [LegacyNullToEmptyString] DOMString cellPadding;
  [CEReactions] attribute [LegacyNullToEmptyString] DOMString cellSpacing;
};