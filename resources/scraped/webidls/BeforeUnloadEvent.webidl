[Exposed=Window]
interface BeforeUnloadEvent : Event {
  attribute DOMString returnValue;
};