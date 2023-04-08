[Exposed=Window]
interface HTMLLabelElement : HTMLElement {
  [HTMLConstructor] constructor();

  readonly attribute HTMLFormElement? form;
  [CEReactions] attribute DOMString htmlFor;
  readonly attribute HTMLElement? control;
};