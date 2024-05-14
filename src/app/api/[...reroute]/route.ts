export async function POST(request: Request) {
	const url = request.url;
	const tail = url.slice(url.indexOf('/api/') + 5);
	const forwarded = `${process.env.API_URL}${tail}`;
	const body = await request.text();

	return await fetch(forwarded, {
		method: 'POST',
		headers: request.headers,
		body: body,
	});
}

export function generateStaticParams() {
	return [];
}
