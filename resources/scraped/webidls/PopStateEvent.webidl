[Exposed=Window]
interface PopStateEvent : Event {
  constructor(DOMString type, optional PopStateEventInit eventInitDict = {});

  readonly attribute any state;
};

dictionary PopStateEventInit : EventInit {
  any state = null;
};