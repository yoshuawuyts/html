[Exposed=(Window)]
interface VisibilityStateEntry : PerformanceEntry {
  readonly attribute DOMString name;                 // shadows inherited name
  readonly attribute DOMString entryType;            // shadows inherited entryType
  readonly attribute DOMHighResTimeStamp startTime;  // shadows inherited startTime
  readonly attribute unsigned long duration;         // shadows inherited duration
};