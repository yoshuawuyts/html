[Global=(Worklet,FakeWorklet),
 Exposed=FakeWorklet,
 SecureContext]
interface FakeWorkletGlobalScope : WorkletGlobalScope {
  undefined registerFake(DOMString type, Function classConstructor);
};