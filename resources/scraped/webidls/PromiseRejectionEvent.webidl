[Exposed=*]
interface PromiseRejectionEvent : Event {
  constructor(DOMString type, PromiseRejectionEventInit eventInitDict);

  readonly attribute object promise;
  readonly attribute any reason;
};

dictionary PromiseRejectionEventInit : EventInit {
  required object promise;
  any reason;
};