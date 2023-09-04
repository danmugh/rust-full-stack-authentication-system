<script lang="ts">
	import { applyAction, enhance, type SubmitFunction } from '$app/forms';
	import { loading } from '$lib/stores/loading.store';
	import { notification } from '$lib/stores/notification.store';
	import { happyEmoji } from '$lib/utils/constant';
	import type { ActionData } from './$types';
	import { receive, send } from '$lib/utils/helpers/animate.crossfade';
	import { page } from '$app/stores';

	export let form: ActionData;

	const handleLogin: SubmitFunction = async () => {
		loading.setLoading(true, 'Please wait while we log you in...');

		return async ({ result }) => {
			loading.setLoading(false);
			if (result.type === 'success' || result.type === 'redirect') {
				$notification = {
					message: `Login successfull ${happyEmoji}...`,
					colorName: `emerald`
				};
			}
			await applyAction(result);
		};
	};

	let message = '';

	if ($page.url.search) {
		message = $page.url.search.split('=')[1].replaceAll('%20', ' ');
	}

	if (message) {
		$notification = {
			message: `${message} ${happyEmoji}...`,
			colorName: 'emerald'
		};
	}
</script>

<svelte:head>
	<title>Auth - Login | Auth Systems with SvelteKit</title>
</svelte:head>

<div class="flex items-center justify-center h-[100vh]">
	<form
		class="w-11/12 md:w-2/3 lg:w-1/3 rounded-xl flex flex-col items-center bg-slate-800 py-6"
		method="POST"
		action="?/login"
		use:enhance={handleLogin}
	>
	<div class="text-center text-2xl font-bold mb-6 text-transparent bg-clip-text bg-gradient-to-t from-sky-600 to-sky-400">Login</div>

		{#if form?.errors}
			{#each form?.errors as error (error.id)}
				<p
					class="text-center text-rose-600"
					in:receive={{ key: error.id }}
					out:send={{ key: error.id }}
				>
					{error.error}
				</p>
			{/each}
		{/if}

		<input
			type="hidden"
			name="next"
			value={$page.url.searchParams.get('next')}
		/>

		<div class="w-3/4 mb-2">
			<input
				type="email"
				name="email"
				id="email"
				class="w-full text-white placeholder:text-slate-600 border-none focus:ring-0 bg-main-color focus:outline-none py-2 px-3 rounded"
				placeholder="Email address"
				required
			/>
		</div>

		<div class="w-3/4 mb-2">
			<input
				type="password"
				name="password"
				id="password"
				class="w-full text-white placeholder:text-slate-600 border-none focus:ring-0 bg-main-color focus:outline-none py-2 px-3 rounded"
				placeholder="Password"
				required
			/>
		</div>

		<p class="mb-4">
			<a href="/auth/password/request-change" class="text-sm text-slate-400"> Forgot password? </a>
		</p>

		<div class="w-3/4">
			<button
				type="submit"
				class="py-2 bg-gradient-to-l from-sky-500 to-sky-800 w-full rounded text-blue-50 font-bold hover:bg-sky-700"
			>
				Login
			</button>
		</div>

		<div class="w-3/4 flex flex-row justify-center my-2">
			<span
				class="text-sm text-sky-400"
				style="text-align: center; margin-top: 0.5rem"
			>
				No account?
				<a
					href="/auth/register"
					class="ml-2 text-slate-400 underline hover:text-sky-400"
				>
					Create an account.
				</a>
			</span>
		</div>
	</form>
</div>