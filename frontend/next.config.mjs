/** @type {import('next').NextConfig} */
const nextConfig = {
    output: 'export',
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
    env: {
        API_URL: "https://zongjian.uniix.dev"
    }
};

export default nextConfig;
