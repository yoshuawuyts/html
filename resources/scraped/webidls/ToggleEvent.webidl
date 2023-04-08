[Exposed=Window]
interface ToggleEvent : Event {
  constructor(DOMString type, optional ToggleEventInit eventInitDict = {});
  readonly attribute DOMString oldState;
  readonly attribute DOMString newState;
};

dictionary ToggleEventInit : EventInit {
  DOMString oldState = "";
  DOMString newState = "";
};