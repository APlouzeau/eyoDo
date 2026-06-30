export default function TaskCard() {
    return (
        <>
            <div>
                <div className="flex flex-col items-center justify-between">Title</div>
                <div className="flex flex-col items-center justify-between">Description</div>
                <div className="flex flex-col items-center justify-between">Date</div>
                <div className="flex flex-col items-center justify-between">Status</div>
                <div className="flex flex-col items-center justify-between">Actions</div>
                <div className="flex flex-col items-center justify-between">Comments</div>
                <div className="flex flex-col items-center justify-between">Attribution</div>
            </div>
        </>
    );
}
