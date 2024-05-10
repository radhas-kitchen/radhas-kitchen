'use client';

import { LoginForm } from '@/components/ui/loginform';
import { useAuth, login } from '@utils/auth';
import { useRouter } from 'next/navigation';
import { useState } from 'react';

export default function Page() {
	const [status, setStatus] = useState('awaiting' as 'awaiting' | 'loading');
	const auth = useAuth();
	const router = useRouter();

	return (
		<div className='w-full h-full content-center' style={{ marginInline: 'auto' }}>
			<LoginForm
				title="Radha's Kitchen"
				body='Please sign in to continue.'
				submit={ (n) => {
					setStatus('loading');
					login(n.email, n.password).then((data) => {
						auth.setToken(data);
						alert('Logged in!');
						router.push('/');
					}).catch(console.error);
				} }
				status={ status }
			/>
		</div>
	);
}
