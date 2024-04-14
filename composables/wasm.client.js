import init, { greets, add } from "~/pkg/wasmfile.js"

export default function useWasm() {
  const greeting = ref("");
  const input1 = ref(0);
  const input2 = ref(0);
  const result = ref(0);

  onMounted(() => {
    if (process.client) {        
        alert("client side running")
        console.log("Running client side, ", process.client);
        init().then(() => {
            greeting.value = greets();
        });  
    }
  });

  function using_rust_add(a, b) {
    console.log("`add` button working");
    result.value = add(a, b)
  }

  return { greeting, input1, input2, result, using_rust_add };
}