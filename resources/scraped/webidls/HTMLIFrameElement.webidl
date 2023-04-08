partial interface HTMLIFrameElement {
  [CEReactions] attribute DOMString align;
  [CEReactions] attribute DOMString scrolling;
  [CEReactions] attribute DOMString frameBorder;
  [CEReactions] attribute USVString longDesc;

  [CEReactions] attribute [LegacyNullToEmptyString] DOMString marginHeight;
  [CEReactions] attribute [LegacyNullToEmptyString] DOMString marginWidth;
};