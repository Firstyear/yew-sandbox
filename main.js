import init, { run_app } from './pkg/yew_sandbox.js';
async function main() {
   await init('/pkg/yew_sandbox_bg.wasm');
   run_app();
}
main()
