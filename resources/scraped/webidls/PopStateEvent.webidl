[Exposed=Window]
interface PopStateEvent : Event {
  constructor(DOMString type, optional PopStateEventInit eventInitDict = {});

  readonly attribute any state;
  readonly attribute boolean hasUAVisualTransition;
};

dictionary PopStateEventInit : EventInit {
  any state = null;
  boolean hasUAVisualTransition = false;
};