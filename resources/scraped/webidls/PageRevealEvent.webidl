[Exposed=Window]
interface PageRevealEvent : Event {
  constructor(DOMString type, optional PageRevealEventInit eventInitDict = {});
  readonly attribute ViewTransition? viewTransition;
};

dictionary PageRevealEventInit : EventInit {
  ViewTransition? viewTransition = null;
};