[Exposed=Window]
interface CustomElementRegistry {
  [CEReactions] undefined define(DOMString name, CustomElementConstructor constructor, optional ElementDefinitionOptions options = {});
  (CustomElementConstructor or undefined) get(DOMString name);
  DOMString? getName(CustomElementConstructor constructor);
  Promise<CustomElementConstructor> whenDefined(DOMString name);
  [CEReactions] undefined upgrade(Node root);
};

callback CustomElementConstructor = HTMLElement ();

dictionary ElementDefinitionOptions {
  DOMString extends;
};