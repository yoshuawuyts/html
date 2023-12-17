[Exposed=Window]
interface CloseWatcher : EventTarget {
  constructor(optional CloseWatcherOptions options = {});

  undefined requestClose();
  undefined close();
  undefined destroy();

  attribute EventHandler oncancel;
  attribute EventHandler onclose;
};

dictionary CloseWatcherOptions {
  AbortSignal signal;
};