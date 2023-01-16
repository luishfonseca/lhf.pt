import { writable, type Updater } from "svelte/store";

export function checkedWritable(fail: string, allowed: string[]) {
    let store = writable(fail);

    return {
        ...store,
        set: (value: string | undefined) => {
            value = (value && allowed.findIndex(v => v === value) >= 0) ? value : fail;
            store.set(value);
        },
        reset: () => {
            store.set(fail);
        },
        update: (fn: Updater<string | undefined>) => store.update(current => {
            let value = fn(current);
            value = (value && allowed.findIndex(v => v === value) >= 0) ? value : fail;
            return value;
        })
    }
}
