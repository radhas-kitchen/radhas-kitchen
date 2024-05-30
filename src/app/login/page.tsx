'use client';

import { Form, FormControl, FormDescription, FormField, FormItem, FormLabel, FormMessage } from '@components/ui/form';
import { Button } from '@components/ui/button';
import { Input } from '@components/ui/input';
import { Spinner } from '@components/ui/spinner';

import { useAuth } from '@utils/auth';

import { useRouter } from 'next/navigation';
import { useState } from 'react';
import Link from 'next/link';

import { zodResolver } from '@hookform/resolvers/zod';
import { useForm } from 'react-hook-form';
import { z } from 'zod';
import { Auth } from '@proto/auth';

const FormSchema = z.object({
	email: z.string().email(),
	password: z.string().min(8).max(64),
});

function LoginForm({ submit }: { submit: (data: z.infer<typeof FormSchema>) => void }) {
	const form = useForm<z.infer<typeof FormSchema>>({ resolver: zodResolver(FormSchema) });

	return (
		<Form { ...form }>
			<form onSubmit={ form.handleSubmit(submit) }>
				<FormField
					control={ form.control }
					name='email'
					render={ ({ field }) => (
						<FormItem className='mb-4'>
							<FormLabel htmlFor='email'>Email</FormLabel>
							<FormControl>
								<Input placeholder='user@example.com' { ...field } />
							</FormControl>
							<FormDescription>
								You will use this to sign in
							</FormDescription>
							<FormMessage />
						</FormItem>
					) }
				/>

				<FormField
					control={ form.control }
					name='password'
					render={ ({ field }) => (
						<FormItem className='mb-4'>
							<FormLabel htmlFor='password'>Password</FormLabel>
							<FormControl>
								<Input type='password' { ...field } />
							</FormControl>
							<FormDescription>
								At least 8 characters
							</FormDescription>
							<FormMessage />
						</FormItem>
					) }
				/>

				<Button type='submit' className='w-full'>
					Sign in &rarr;
				</Button>

				<div className='flex justify-stretch items-center my-4'>
					<div className='border-[1px] w-full h-[1px]' />
				</div>

				<Button asChild className='w-full'>
					<Link href='/signup'>Go to Sign Up</Link>
				</Button>
			</form>
		</Form>
	);
}

enum FormStatus {
	Awaiting,
	Loading,
	Success,
	Error,
}

export default function Page() {
	const [status, setStatus] = useState(FormStatus.Awaiting);
	const [error, setError] = useState('');
	const auth = useAuth();
	const router = useRouter();

	const submit = async (data: z.infer<typeof FormSchema>) => {
		setStatus(FormStatus.Loading);

		await Auth.Login({ ...data }).then((response) => {
			auth.setToken(response);
			setStatus(FormStatus.Success);
		}).catch((error) => {
			setError(error);
			setStatus(FormStatus.Error);
		});
	};

	return (
		<div className='w-full h-full flex flex-col justify-center'>
			<div className='max-w-md w-96 mx-auto rounded-md md:rounded-2xl pt-4 px-4 shadow-input bg-white dark:bg-black'>
				<h2 className='font-bold text-xl text-neutral-800 dark:text-neutral-200 mb-4'>
					Radha's Kitchen
				</h2>

				{(() => {
					switch (status) {
						case FormStatus.Awaiting: {
							return (
								<LoginForm submit={ submit } />
							);
						}
						case FormStatus.Loading: {
							return (
								<div className='m-auto'>
									<Spinner />
								</div>
							);
						}
						case FormStatus.Success: {
							return router.push('/') as never;
						}
						case FormStatus.Error: {
							return (
								<div className='m-auto'>
									<h1 className='text-center text-lg'>Error</h1>
									<p className='text-center text-sm text-zinc-300'>{ error }</p>
									<Button className='w-full mt-4' onClick={ () => setStatus(FormStatus.Awaiting) }>
										Try Again
									</Button>
								</div>
							);
						}
					}
				})()}

				<div className='bg-gradient-to-r from-transparent via-neutral-300 dark:via-neutral-700 to-transparent mt-4 h-[1px] w-full' />
			</div>
		</div>
	);
}
