import React, { useEffect, useRef, useState } from "react";
import init, { Gl2d, GlowBackend, Io, license } from "@crate/gl2d/pkg";
import { H5, Pre } from "@blueprintjs/core";

export const App: React.FC = () => {
  return (
    <>
      <div className="m-6">
        <p>Trackpad: swipe, pinch</p>
        <p>Mouse: scroll with ctrl or shift</p>
      </div>
      <GlCanvas />
    </>
  );
};

type GlCanvasProps = {};
export const GlCanvas: React.FC<GlCanvasProps> = () => {
  const [licenseNotice, setLicenseNotice] = useState("");
  const wrapper = useRef<HTMLDivElement>(null);
  const canvas = useRef<HTMLCanvasElement>(null);
  useEffect(() => {
    let isUnmounted = false;
    const webgl = canvas.current!.getContext("webgl2")!;
    let io: Io | null = null;
    init().then(() => {
      setLicenseNotice(license());
      const backend = new GlowBackend(webgl);
      const gl2d = new Gl2d(backend);
      io = new Io();
      const loop = () => {
        if (isUnmounted) {
          gl2d.free();
          return;
        }
        requestAnimationFrame(loop);
        const width = wrapper.current!.clientWidth;
        const height = wrapper.current!.clientHeight;
        canvas.current!.width = width * window.devicePixelRatio;
        canvas.current!.height = height * window.devicePixelRatio;
        io!.setScreenSize(width, height, window.devicePixelRatio);
        gl2d.begin_frame(io!);
        gl2d.draw();
      };
      requestAnimationFrame(loop);
    });
    const currentCanvas = canvas.current!;
    const onWheel = function (this: HTMLCanvasElement, e: WheelEvent) {
      e.preventDefault();
      if (!io) {
        return;
      }
      if (e.ctrlKey) {
        io.pinch += e.deltaY;
      } else if (e.shiftKey) {
        io.wheelX += e.deltaY;
        io.wheelY += e.deltaX;
      } else {
        io.wheelX += e.deltaX;
        io.wheelY += e.deltaY;
      }
    };
    const onMouseMove = function (this: HTMLCanvasElement, e: MouseEvent) {
      e.preventDefault();
      if (!io) {
        return;
      }
      const rect = canvas.current!.getBoundingClientRect();
      io.mouseX = e.clientX - rect.left;
      io.mouseY = e.clientY - rect.top;
    };
    currentCanvas.addEventListener("wheel", onWheel);
    currentCanvas.addEventListener("mousemove", onMouseMove);
    return () => {
      isUnmounted = true;
      currentCanvas.removeEventListener("wheel", onWheel);
      currentCanvas.removeEventListener("mousemove", onMouseMove);
    };
  }, []);
  return (
    <>
      <div
        ref={wrapper}
        style={{ width: "1000px", height: "1000px" }}
        className="bg-gray-300 m-6"
      >
        <canvas ref={canvas} className="block h-full w-full" />
      </div>
      <div className="m-6">
        <H5>License Notice</H5>
        <Pre>{licenseNotice}</Pre>
      </div>
    </>
  );
};
