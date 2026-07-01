"use client";

import { useEffect } from "react";
import { createPortal } from "react-dom";
interface ModalProps {
    isOpen: boolean;
    onClose: () => void;
    children: React.ReactNode;
}

export default function Modal({ isOpen, onClose, children }: ModalProps) {
    useEffect(() => {
        if (isOpen) {
            document.body.style.overflow = "hidden";
        }
        return () => {
            document.body.style.overflow = "auto";
        };
    }, [isOpen]);

    if (!isOpen) return null;

    return createPortal(
        <div onClick={onClose} className="fixed inset-0 bg-black/10 backdrop-blur-sm flex justify-center items-center">
            <div className="relative rounded shadow-lg" onClick={(e) => e.stopPropagation()}>
                <button onClick={onClose} className="absolute top-2 right-2 bg-red-500 text-white px-4 py-2 rounded">
                    Close
                </button>
                {children}
            </div>
        </div>,
        document.body,
    );
}
