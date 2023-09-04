<div class="w-full flex flex-col md:flex-row justify-start items-center md:items-start my-6 md:my-0">
    <div class="w-full md:w-[70%] h-full p-5 sm:p-20">
        <div class="text-5xl font-bold text-transparent bg-clip-text bg-gradient-to-r from-blue-200 to-cyan-200">
            Welcome to Rust Auth App!
        </div>
        <div class="my-4 text-xl">
            Are you ready to take your web security to the next level? Introducing my latest project, a robust User Authentication System built in Rust. With a focus on performance, security, and user experience! This project has been an incredible learning experience, inspired
            by <a class="text-transparent bg-clip-text bg-gradient-to-r from-blue-200 to-cyan-200" href="https://dev.to/sirneij/full-stack-authentication-system-using-rust-actix-web-and-sveltekit-1cc6" target="_blank">John Idogun's series of Articles.</a>
        </div>
        <div class="my-4 text-xl text-transparent bg-clip-text bg-gradient-to-r from-blue-200 to-cyan-200">Let's dive into the tech stack and key highlights:</div>
        <div class="mt-8 mb-4 text-xl">🚀 Tech Stack:</div>
        <ul class="ml-10" style="list-style-type:square">
            <li>
                <div class="my-4 text-xl text-transparent bg-clip-text bg-gradient-to-r from-blue-200 to-cyan-200">Backend :</div>
                <ul class="ml-12 " style="list-style-type:disc">
                    <li class="text-lg">Actix web (v4) - The foundation of our web framework</li>
                    <li class="text-lg">Tokio v1 - Empowering asynchronous operations</li>
                    <li class="text-lg">Serde v1 - Handling data serialization and deserialization</li>
                    <li class="text-lg">MiniJinja v0.32 - Crafting dynamic templates</li>
                    <li class="text-lg">SQLx v0.6 - Asynchronous SQL toolkit for Rust</li>
                    <li class="text-lg">PostgreSQL v15 - A reliable database</li>
                    <li class="text-lg">Redis - Enabling token expiration and more</li>
                </ul>
            </li>
            <li>
                <div class="my-4 text-xl text-transparent bg-clip-text bg-gradient-to-r from-blue-200 to-cyan-200">Frontend :</div>
                <ul class="ml-12" style="list-style-type:disc">
                    <li class="text-lg">SvelteKit v1 - Our frontend framework</li>
                    <li class="text-lg">Pure CSS3 and TailwindCSS v3.3 - Stylish and responsive designs</li>
                    <li class="text-lg">TypeScript v5 - Enhancing type safety</li>
                    <li class="text-lg">HTML5 - Building the structural foundation</li>
                </ul>
            </li>
        </ul>
        <div class="mt-8 mb-4 text-xl">🌟 Key Features:</div>
        <ul class="ml-20" style="list-style-type:disc">
            <li class="text-lg">✅ Secure Email/Password Authentication</li>
            <li class="text-lg">⏱ Token Expiration Management with Redis</li>
            <li class="text-lg">🚀 Outstanding Performance</li>
            <li class="text-lg">🔒 Robust Data Security</li>
            <li class="text-lg">🌐 User-Friendly UI Powered by SvelteKit</li>
        </ul>
        <div
            class="mt-10 mb-8 text-xl text-transparent bg-clip-text bg-gradient-to-r from-blue-200 to-cyan-200">📦 You can explore the full source code and documentation on
            <a
                class="underline text-white"
                href="https://dev.to/sirneij/full-stack-authentication-system-using-rust-actix-web-and-sveltekit-1cc6"
                target="_blank"
                >GitHub
            </a>
        </div>
        <div class="h-[5px] md:h-[40px]" />
    </div>
    <div class="w-full md:h-[100vh] md:w-[30%] py-10 md:py-20 pr-20 pl-6 flex justify-center md:justify-end items-center md:items-start">
        <a href="/auth/login">
            <button class="bg-transparent hover:bg-gradient-to-r from-blue-200 to-cyan-200 text-white font-semibold hover:text-black py-2 px-4 border border-white hover:border-transparent rounded">
                Explore the App
            </button>
        </a>
    </div>
</div>

<style lang="postcss">
    :global(html) {
        background: bg-gradient-to-r from-blue-800 to-indigo-900;
        color: theme(colors.white)
    }
</style>