import init, { run_app } from '../frontend/pkg/frontend';
async function main() {
    await init('/frontend/pkg/frontend_bg.wasm');
    run_app();
}
main()
