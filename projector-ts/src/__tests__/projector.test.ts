import { Operation } from "../config";
import Projector from "../projector";
import { expect, test } from "bun:test";

function getData() {
    return {
        projector: {
            "/": {
                "foo": "bar1",
                "fem": "is_great",
            },
            "/foo": {
                "foo": "bar2",
            },
            "/foo/bar": {
                "foo": "bar3",
            },
        }
    }
}

function getProjector(pwd: string): Projector {
    return new Projector({
        args: [],
        operation: Operation.Print,
        pwd,
        config: "Hello, frontend masters"
    }, getData())
}

test("getValueAll", function () {
    const proj = getProjector("/foo/bar")
    expect(proj.getValueAll()).toEqual({
        "fem": "is_great",
        "foo": "bar3"
    })
})

test("getValue", function () {
    const proj = getProjector("/foo/bar")
    expect(proj.getValue("foo")).toEqual(
        "bar3"
    )
})
