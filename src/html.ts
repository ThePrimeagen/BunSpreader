
export default {
    port: 3000,
    async fetch(request: Request) {
        const params = request.url.split("json/")[1];
        const [time_in_queue] = params.split("/").map(x => +x);
        return new Response(`time in queue will be ${time_in_queue}`);
    },
};
