import init, { run_app } from '../pkg';
async function main() {
    await init('pkg/frontend_bg.wasm');
    run_app();
}
main()
