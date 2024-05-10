// import { internalIpV4 } from 'internal-ip';

/** @type {import('next').NextConfig} */
const nextConfig = {
	output: 'export',
	images: {
		unoptimized: true,
	},
	assetPrefix: 'https://56fd-66-109-53-198.ngrok-free.app',
};

export default nextConfig;
