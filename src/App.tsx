import { useKeyDownEvent } from "@solid-primitives/keyboard";
import { tauri } from "@tauri-apps/api";
import { exit } from "@tauri-apps/api/process";
import { createEffect, For, onMount } from "solid-js";
import { createStore } from "solid-js/store";

interface ListItem {
  name: string;
  exec: string;
  mode: number;
}

function App() {
  let [items, setItems] = createStore([] as ListItem[]);

  let call = async (pat: string) => {
    return await tauri.invoke("call", { pat });
  };

  let exec = async (mode: number, command: string) => {
    return await tauri.invoke("exec", { mode, command });
  };

  let selected_index = 0;

  let input: any;

  onMount(() => {
    input.focus();
  });

  const event = useKeyDownEvent();

  let get_selected_btn = () => {
    return window.document.getElementById("btn-" + selected_index);
  };

  createEffect(() => {
    const e = event();
    if (e) {
      if (e.key == "Enter") {
        e.preventDefault();
        let btn = get_selected_btn();
        btn?.click();
      }
      if (e.key == "ArrowDown") {
        e.preventDefault();
        let old_btn = get_selected_btn();
        old_btn?.classList.replace("border-red", "border-cyan");
        selected_index += 1;
        let new_btn = get_selected_btn();
        new_btn?.classList.replace("border-cyan", "border-red");
        new_btn?.scrollIntoView();
      }
      if (e.key == "ArrowUp") {
        e.preventDefault();
        let old_btn = get_selected_btn();
        old_btn?.classList.replace("border-red", "border-cyan");
        if (selected_index > 0) {
          selected_index -= 1;
        }
        let new_btn = get_selected_btn();
        new_btn?.classList.replace("border-cyan", "border-red");
        new_btn?.scrollIntoView();
      }
      if (e.key == "Escape") {
        e.preventDefault();
        exit(0);
      }
    }
  });

  return (
    <div class="h-screen w-screen border border-yellow rounded-xl flex flex-col overflow-hidden pb-2">
      <div class="flex justify-center h-1/4">
        <input
          ref={input}
          class="bg-yellow border-none focus:border-none rounded-xl m-2 p-1 selection:bg-red"
          onInput={(e) => {
            let value = e.target.value;
            call(value)
              .then((result) => {
                let r = result as ListItem[];
                setItems(r);
              })
              .catch((err) => console.error(err))
              .then(() => {
                selected_index = 0;
                let btn = get_selected_btn();
                btn?.classList.replace("border-cyan", "border-red");
              });
          }}
        />
      </div>
      <div class="flex justify-center overflow-auto">
        <div class="flex flex-col text-white text-nowrap font-bold">
          <For each={items}>
            {(item, index) => (
              <button
                id={"btn-" + index()}
                class="border rounded-xl py-1 px-5 m-1 border-cyan"
                onClick={() => {
                  exec(item.mode, item.exec)
                    .then(() => {
                      exit(0);
                    })
                    .catch((err) => {
                      console.error(err);
                    });
                }}
              >
                {item.name}
              </button>
            )}
          </For>
        </div>
      </div>
    </div>
  );
}

export default App;
