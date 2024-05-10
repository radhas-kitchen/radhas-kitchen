'use client';

import { ensureToken, useAuth } from '@utils/auth';

export default function Index() {
	const auth = useAuth();
	const token = ensureToken(auth);

	if (!token) return <></>;

	return (
		<div className='w-full h-full content-center' style={{ marginInline: 'auto' }}>
			Main Page
		</div>
	);
}
