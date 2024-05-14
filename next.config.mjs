import { internalIpV4 } from 'internal-ip';

const prod = process.env.NODE_ENV === 'production';

/** @type {import('next').NextConfig} */
const nextConfig = {
	output: 'export',
	images: {
		unoptimized: true,
	},
	assetPrefix: prod ? `http://${'192.168.1.94'}:3000/` : undefined,
};

export default nextConfig;
