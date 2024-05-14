'use client';

import { Form, FormStatus, QuestionKind } from '@/components/ui/form';
import { Label } from '@/components/ui/label';
import { useAuth } from '@utils/auth';
import { user } from '@utils/reapi';
import { useRouter } from 'next/navigation';
import { useState } from 'react';

export default function Page() {
	const [status, setStatus] = useState(FormStatus.Awaiting);
	const [message, setMessage] = useState('' as string | React.ReactNode);
	const auth = useAuth();
	const router = useRouter();

	return (
		<div className='w-full h-full content-center' style={{ marginInline: 'auto' }}>
			<Form
				title="Radha's Kitchen"
				body={ (
					<>
						Please login to continue.
						<br />

						Don't have an account? &nbsp;

						<a href='/signup'>Sign up</a>
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
						key: 'password',
						display: 'Password',
						kind: QuestionKind.Input,
						as: 'password',
					},
				] }
				submit={ (n) => {
					setStatus(FormStatus.Loading);

					if (!n.email || !n.password) {
						setStatus(FormStatus.Message);
						setMessage('No email or password provided');
						return;
					}

					user.login(n.email, n.password).then((data) => {
						auth.setToken(data);
						router.push('/');
					}).catch((data: Error) => {
						setMessage((
							<>
								<Label>
									{data.message}
									<br />
								</Label>
								<a href='/'>
									Go back
								</a>
							</>
						));
						setStatus(FormStatus.Message);
					});
				} }
				status={ status }
				message={ message }
				submit_text='Sign In'
			/>
		</div>
	);
}
