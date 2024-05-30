import { useEffect, useState } from 'react';

export function useAsync<T>(fn: () => Promise<T>, setLoading: (l: boolean) => void, setError: (e: Error | undefined) => void): [T | undefined, () => void] {
	const [data, setData] = useState<T | undefined>(undefined);
	const [reload, setReload] = useState(true);

	useEffect(() => {
		if (!reload) return;

		setLoading(true);
		setError(undefined);
		setData(undefined);

		fn().then((data) => {
			setData(data);
			setLoading(false);
		}).catch((error) => {
			setError(error);
			setLoading(false);
		});

		setReload(false);
	}, [reload]);

	return [data, () => setReload(true)];
}
