// TODO: I know i don't have to include them, but it seems to break on docker.
import React from "react";

import hasBackground, { ItemConfig } from "./item-config";

function renderInners(props: ItemConfig) {
    if (props.depth === 0) {
        return props.bit;
    }

    return (
        <>
            <Item {...props} depth={props.depth - 1} girth={props.girth - 3} />
        </>
    );
}

export default function Item(props: ItemConfig) {
    const isEven = Boolean(props.depth & 0x1);
    let backgroundColor = "#ff9500";
    if (isEven === hasBackground(props)) {
        backgroundColor = "#0385ff";
    }

    const styles = {
        width: `${props.girth}px`,
        height: "69px",
        border: "1px solid #DDD",
        backgroundColor,
    };

    return <div style={styles}>{renderInners(props)}</div>;
}
