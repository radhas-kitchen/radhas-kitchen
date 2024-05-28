import { internalIpV4 } from 'internal-ip';

const prod = process.env.NODE_ENV === 'production';

/** @type {import('next').NextConfig} */
const nextConfig = {
	output: 'export',
	images: {
		unoptimized: true,
	},
	assetPrefix: !prod ? `http://${'localhost'}:3000/` : undefined,
};

export default nextConfig;
