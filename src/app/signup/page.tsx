'use client';

import { Button } from '@/components/ui/button';
import { Form, FormStatus, QuestionKind } from '@/components/ui/form';
import { Label } from '@/components/ui/label';
import { UserKind, signup } from '@/utils/reapi/user';
import Link from 'next/link';
import { useState } from 'react';

import '@/app/globals.css';

export default function Page() {
	const [status, setStatus] = useState(FormStatus.Awaiting);
	const [message, setMessage] = useState('' as string);

	return (
		<div className='w-full h-full content-center' style={{ marginInline: 'auto' }}>
			<Form
				title="Radha's Kitchen"
				body={ (
					<>
						Please sign up to continue.
						<br />
						Have an account? &nbsp;

						<a href='/login'>Login</a>
					</>
				) }
				questions={ [
					{
						key: 'email',
						display: 'Email Address',
						kind: QuestionKind.Input,
						as: 'email',
						placeholder: 'user@example.com',
					},
					{
						key: 'name',
						display: 'Display Name',
						kind: QuestionKind.Input,
						placeholder: 'My Wonderful Restaurant',
					},
					{
						key: 'password',
						display: 'Password',
						kind: QuestionKind.Input,
						as: 'password',
					},
					{
						key: 'kind',
						display: 'Account Type',
						kind: QuestionKind.Select,
						options: [
							{
								value: 'Driver',
								display: 'Driver',
							},
							{
								value: 'Restaurant',
								display: 'Restaurant',
							},
							{
								value: 'Farm',
								display: 'Farm',
							},
						],
					},
				] }
				submit={ (n) => {
					setStatus(FormStatus.Loading);

					if (!n.email || !n.password || !n.name || !n.kind) {
						setStatus(FormStatus.Message);
						setMessage('No email, password, name, or kind provided');
						return;
					}

					const kind = UserKind[n.kind as keyof typeof UserKind];

					signup(n.email, n.password, n.name, kind).then(() => {
						setMessage('Success: Account created');
						setStatus(FormStatus.Message);
					}).catch((data: Error) => {
						setMessage(`Error: ${data.message}`);
						setStatus(FormStatus.Message);
					});
				} }
				status={ status }
				message={ (
					<>
						<Label className='my-8 mx-auto'>
							{ message }
						</Label>
						<Button className='w-full mt-4' asChild>
							<Link href='/'>
								Go Home
							</Link>
						</Button>
					</>
				) }
				submit_text='Sign Up'
			/>
		</div>
	);
}
