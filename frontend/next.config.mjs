/** @type {import('next').NextConfig} */
const nextConfig = {
    images: {
        remotePatterns: [
            {
                protocol: 'https',
                hostname: 'hdslb.com',
                port: '',
                pathname: '**',
            },
        ],
    },
};

export default nextConfig;
