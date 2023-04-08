partial interface HTMLImageElement {
  [CEReactions] attribute DOMString name;
  [CEReactions] attribute USVString lowsrc;
  [CEReactions] attribute DOMString align;
  [CEReactions] attribute unsigned long hspace;
  [CEReactions] attribute unsigned long vspace;
  [CEReactions] attribute USVString longDesc;

  [CEReactions] attribute [LegacyNullToEmptyString] DOMString border;
};