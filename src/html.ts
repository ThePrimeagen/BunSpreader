const queue = [];

export default {
    port: 3000,
    async fetch(request: Request) {
        const params = request.url.split("json/")[1];
        let time_in_queue = 5000;
        if (params) {
            time_in_queue = params.split("/").map(x => +x)[0];
        }

        try {
            const json = await request.json();
            queue.push({
                json,
                time: Date.now() + time_in_queue,
            });
        } catch (e) {
            console.error("unable to parse json", e);
        }

        const now = Date.now();
        while (queue.length > 0 && queue[0].time < now) {
            queue.shift();
        }

        return new Response(`time in queue will be ${time_in_queue}`);
    },
};
