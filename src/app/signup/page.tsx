'use client';

import { Form, FormControl, FormDescription, FormField, FormItem, FormLabel, FormMessage } from '@/components/ui/form';
import { Select, SelectTrigger, SelectContent, SelectItem } from '@/components/ui/select';
import { Input } from '@/components/ui/input';
import { Button } from '@/components/ui/button';
import { Spinner } from '@components/ui/spinner';

import { useAuth } from '@utils/auth';

import Link from 'next/link';
import { useRouter } from 'next/navigation';
import { useEffect, useState } from 'react';

import { zodResolver } from '@hookform/resolvers/zod';
import { useForm } from 'react-hook-form';
import { z } from 'zod';
import { invoke } from '@tauri-apps/api/core';

const FormSchema = z.object({
	email: z.string().email(),
	password: z.string().min(8).max(64),
	name: z.string().min(2).max(32),
	kind: z.enum([
		'Provider',
		'Driver',
		'Consumer',
	]),
	address: z.string().min(2).optional(),
}).superRefine((data, ctx) => {
	if (data.kind !== 'Driver' && !data.address) {
		ctx.addIssue({
			code: z.ZodIssueCode.custom,
			message: 'Address is required for non-driver accounts.',
			path: ['address'],
		});
	}

	if (data.kind == 'Driver' && data.address) {
		ctx.addIssue({
			code: z.ZodIssueCode.custom,
			message: 'Address cannot be used for driver accounts',
			path: ['address'],
		});
	}

	return z.NEVER;
});

function LoginForm({ submit }: { submit: (data: z.infer<typeof FormSchema>) => void }) {
	const form = useForm<z.infer<typeof FormSchema>>({ resolver: zodResolver(FormSchema) });

	return (
		<Form { ...form }>
			<form onSubmit={ form.handleSubmit(submit) }>
				<div className='grid gap-2 grid-cols-2'>
					<FormField
						control={ form.control }
						name='email'
						render={ ({ field }) => (
							<FormItem>
								<FormLabel htmlFor={ field.name }>Email</FormLabel>
								<FormControl>
									<Input placeholder='user@example.com' { ...field } />
								</FormControl>
								<FormMessage />
							</FormItem>
						) }
					/>

					<FormField
						control={ form.control }
						name='password'
						render={ ({ field }) => (
							<FormItem>
								<FormLabel htmlFor={ field.name }>Password</FormLabel>
								<FormControl>
									<Input type='password' { ...field } />
								</FormControl>
								<FormMessage />
							</FormItem>
						) }
					/>

					<FormField
						control={ form.control }
						name='name'
						render={ ({ field }) => (
							<FormItem>
								<FormLabel htmlFor={ field.name }>Name</FormLabel>
								<FormControl>
									<Input placeholder='John Doe' { ...field } />
								</FormControl>
								<FormDescription>
									Business name for farms and resteraunts
								</FormDescription>
								<FormMessage />
							</FormItem>
						) }
					/>

					<FormField
						control={ form.control }
						name='kind'
						render={ ({ field }) => (
							<FormItem>
								<FormLabel htmlFor={ field.name }>Kind</FormLabel>
								<div style={{ marginTop: '0.625rem' }}>
									<Select onValueChange={ field.onChange } defaultValue={ field.value }>
										<FormControl>
											<SelectTrigger className='bg-zinc-800'>
												<Input placeholder='Choose an option' { ...field } className='-ml-3' />
											</SelectTrigger>
										</FormControl>
										<SelectContent>
											<SelectItem value='Provider'>Restaurant</SelectItem>
											<SelectItem value='Driver'>Driver</SelectItem>
											<SelectItem value='Consumer'>Farm</SelectItem>
										</SelectContent>
									</Select>
								</div>
								<FormDescription style={{ marginTop: '0.625rem' }}>
									What kind of account are you creating?
								</FormDescription>
								<FormMessage />
							</FormItem>
						) }
					/>
				</div>

				<FormField
					control={ form.control }
					name='address'
					render={ ({ field }) => (
						<FormItem className='mt-2'>
							<FormLabel htmlFor={ field.name }>Address</FormLabel>
							<FormControl>
								<Input placeholder='1234 Elm St' { ...field } />
							</FormControl>
							<FormDescription>
								Dropoff and pickup locations for farms and resteraunts
							</FormDescription>
							<FormMessage />
						</FormItem>
					) }
				/>

				<Button type='submit' className='w-full mt-8'>
					Sign Up &rarr;
				</Button>

				<div className='flex justify-stretch items-center my-4'>
					<div className='border-[1px] w-full h-[1px]' />
					<p className='mx-2 text-nowrap text-zinc-400 text-sm'>have an account?</p>
					<div className='border-[1px] w-full h-[1px]' />
				</div>

				<Button asChild className='w-full'>
					<Link href='/login'>Go to Login</Link>
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
	const router = useRouter();

	const submit = async (data: z.infer<typeof FormSchema>) => {
		setStatus(FormStatus.Loading);

		let kind: any = null;

		if (data.kind === 'Provider') kind = { Provider: { location: data.address! } };
		else if (data.kind === 'Consumer') kind = { Consumer: { location: data.address! } };

		await invoke('grpc_create_user', {
			request: {
				email: data.email,
				password: data.password,
				name: data.name,
				kind,
			},
		}).catch((e) => {
			setError(e);
			setStatus(FormStatus.Error);
		}).then(() => {
			setStatus(FormStatus.Success);
		});
	};

	return (
		<div className='w-full h-full flex flex-col justify-center'>
			<div className='max-w-md w-96 mx-auto rounded-md md:rounded-2xl pt-4 px-4 shadow-input bg-white dark:bg-black'>
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
							return (
								<div className='m-auto'>
									<h1 className='text-center text-lg'>Success!</h1>
									<Button className='w-full mt-4' onClick={ () => router.push('/login') }>
										Go to Login &rarr;
									</Button>
								</div>
							);
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
