import React, { DragEvent, MouseEvent, useCallback, useState } from 'react'
import AutoSizer, { Size } from 'react-virtualized-auto-sizer'
import ReactFlow, {
  Background,
  BackgroundVariant,
  Connection,
  ControlButton,
  Controls,
  Edge,
  MiniMap,
  Node,
  OnEdgesChange,
  OnNodesChange,
  Position,
  ReactFlowInstance,
  ReactFlowProvider,
  addEdge,
  useEdgesState,
  useNodesState,
  useOnSelectionChange,
} from 'reactflow'
import 'reactflow/dist/style.css'
import './FlowEditor.css'
import Dagre from '@dagrejs/dagre'
import { v4 as uuid } from 'uuid'
import ComponentForm from './ComponentForm'

type CompType = 'source' | 'sink' | 'pipe'
export type BuiltInComp = {
  id: string
  label: string
  type: CompType
  inputForm: object
  outputForm: object
}

const dagreGraph = new Dagre.graphlib.Graph()
dagreGraph.setDefaultEdgeLabel(() => ({}))

const defNodeWidth = 172
const defNodeHeight = 36
type LayoutDir = 'TB' | 'LR'

function NodeList<T extends { id: string; type: CompType; label: string }>({
  nodes,
  onClick,
}: {
  nodes: T[]
  onClick?: (node: T, e: MouseEvent) => void
}) {
  const onDragStart = (event: DragEvent, nodeType: T) => {
    event.dataTransfer.setData('application/reactflow', nodeType.id)
    event.dataTransfer.effectAllowed = 'move'
  }
  return (
    <div className="FlowEditor">
      <p>Drag nodes onto the grid</p>
      <ul className="dndnodelist">
        {nodes.map(n => (
          <li
            className={`dndnode react-flow__node react-flow__node-${getNodeType(n)}`}
            key={n.id}
            onDragStart={e => onDragStart(e, n)}
            onClick={e => onClick?.(n, e)}
            draggable
          >
            {n.label}
            {n.type === 'source' || n.type === 'pipe' ? (
              <div
                className={`react-flow__handle react-flow__handle-right`}
              ></div>
            ) : null}
            {n.type === 'sink' || n.type === 'pipe' ? (
              <div
                className={`react-flow__handle react-flow__handle-left`}
              ></div>
            ) : null}
          </li>
        ))}
      </ul>
    </div>
  )
}

function getNodeType<T extends { type: CompType }>(data: T) {
  return data.type === 'source'
    ? 'input'
    : data.type === 'sink'
      ? 'output'
      : 'default'
}

function makeNode<T extends { type: CompType }>(dir: LayoutDir, node: Node<T>) {
  const isHorizontal = dir === 'LR'
  const targetPos = isHorizontal ? Position.Left : Position.Top
  const sourcePos = isHorizontal ? Position.Right : Position.Bottom

  return {
    ...node,
    type: getNodeType(node.data),
    sourcePosition: sourcePos,
    targetPosition: targetPos,
  }
}

function getLayoutedElements<T extends { type: CompType }>(
  nodes: Node<T>[],
  edges: Edge[],
  dir: LayoutDir = 'LR',
) {
  dagreGraph.setGraph({ rankdir: dir })

  nodes.forEach(n =>
    dagreGraph.setNode(n.id, {
      width: n.width ?? defNodeWidth,
      height: n.height ?? defNodeHeight,
    }),
  )
  edges.forEach(e => dagreGraph.setEdge(e.source, e.target))

  Dagre.layout(dagreGraph)

  return {
    nodes: nodes.map((n): Node<T> => {
      const nodeWithPosition = dagreGraph.node(n.id)

      return makeNode(dir, {
        ...n,
        position: {
          x: nodeWithPosition.x - nodeWithPosition.width / 2,
          y: nodeWithPosition.y - nodeWithPosition.height / 2,
        },
      })
    }),
    edges,
  }
}

function FlowPanel({
  availableNodes,
  dir,
  setDir,
  nodes,
  onNodesChange,
  setNodes,
  edges,
  onEdgesChange,
  setEdges,
}: {
  availableNodes: BuiltInComp[]
  dir: LayoutDir
  setDir: (dir: LayoutDir) => void
  nodes: Node<BuiltInComp>[]
  onNodesChange: OnNodesChange
  setNodes: React.Dispatch<React.SetStateAction<Node<BuiltInComp>[]>>
  edges: Edge[]
  onEdgesChange: OnEdgesChange
  setEdges: ReturnType<typeof useEdgesState>[1]
}) {
  const [reactFlowInstance, setReactFlowInstance] =
    useState<ReactFlowInstance<BuiltInComp>>()

  const onConnect = useCallback(
    (params: Edge | Connection) => setEdges(eds => addEdge(params, eds)),
    [setEdges],
  )

  const onDragOver = useCallback((event: DragEvent) => {
    event.preventDefault()
    event.dataTransfer.dropEffect = 'move'
  }, [])

  const onDrop = useCallback(
    (event: DragEvent) => {
      event.preventDefault()
      const id = event.dataTransfer.getData('application/reactflow')
      if (!id || !reactFlowInstance) {
        return
      }
      const foundNode = availableNodes.find(n => n.id === id)
      if (!foundNode) {
        return
      }

      const position = reactFlowInstance.screenToFlowPosition({
        x: event.clientX,
        y: event.clientY,
      })
      const newNode = makeNode(dir, {
        id: uuid(),
        position,
        data: { ...foundNode },
      })

      setNodes(nds => nds.concat(newNode))
    },
    [availableNodes, dir, reactFlowInstance],
  )

  const onLayout = useCallback(
    (dir: LayoutDir) => {
      const { nodes: newNodes, edges: newEdges } = getLayoutedElements(
        nodes,
        edges,
        dir,
      )

      setDir(dir)
      setNodes(newNodes)
      setEdges(newEdges)
    },
    [nodes, edges],
  )

  return (
    <div style={{ flex: '1 1 auto' }}>
      <AutoSizer>
        {({ width, height }: Size) => (
          <div style={{ width, height }}>
            <ReactFlow
              nodes={nodes}
              edges={edges}
              onNodesChange={onNodesChange}
              onEdgesChange={onEdgesChange}
              onConnect={onConnect}
              onInit={setReactFlowInstance}
              onDrop={onDrop}
              onDragOver={onDragOver}
              fitView
            >
              <Controls showInteractive={false}>
                <ControlButton
                  onClick={() => onLayout('TB')}
                  aria-label="Vertical Layout"
                  title="Vertical Layout"
                >
                  ↓
                </ControlButton>
                <ControlButton
                  onClick={() => onLayout('LR')}
                  aria-label="Horizontal Layout"
                  title="Horizontal Layout"
                >
                  →
                </ControlButton>
              </Controls>
              <MiniMap />
              <Background variant={BackgroundVariant.Dots} gap={12} size={1} />
            </ReactFlow>
          </div>
        )}
      </AutoSizer>
    </div>
  )
}

export function FlowEditorInner({
  style,
  availableNodes,
  ...rest
}: {
  style?: React.CSSProperties
  availableNodes: BuiltInComp[]
}) {
  const [nodes, setNodes, onNodesChange] = useNodesState(
    [] as Node<BuiltInComp>[],
  )
  const [edges, setEdges, onEdgesChange] = useEdgesState([])
  const [dir, setDir] = useState<LayoutDir>('LR')
  const [selectedNodes, setSelectedNodes] = useState([] as Node<BuiltInComp>[])

  useOnSelectionChange({
    onChange: ({ nodes }) => {
      setSelectedNodes(nodes)
    },
  })

  const addNode = useCallback(
    (newComp: BuiltInComp) => {
      // get the furthest node
      const xs = nodes.map(n => n.position.x)
      const ys = nodes.map(n => n.position.y)

      const avg = (ns: number[], def: number) =>
        ns.length === 0 ? def : ns.reduce((acc, c) => acc + c, 0) / ns.length

      setNodes([
        ...nodes,
        makeNode(dir, {
          id: uuid(),
          position:
            dir === 'LR'
              ? {
                  x:
                    xs.length === 0
                      ? defNodeWidth
                      : Math.max(...xs) + defNodeWidth,
                  y: avg(ys, defNodeHeight),
                }
              : {
                  y:
                    ys.length === 0
                      ? defNodeHeight
                      : Math.max(...ys) + defNodeHeight * 2,
                  x: avg(xs, defNodeWidth),
                },
          data: { ...newComp },
        }),
      ])
    },
    [nodes, dir],
  )

  return (
    <div style={{ display: 'flex', flexFlow: 'row', ...style }} {...rest}>
      <NodeList nodes={availableNodes} onClick={n => addNode(n)} />
      <FlowPanel
        availableNodes={availableNodes}
        edges={edges}
        setEdges={setEdges}
        onEdgesChange={onEdgesChange}
        nodes={nodes}
        setNodes={setNodes}
        onNodesChange={onNodesChange}
        dir={dir}
        setDir={setDir}
      />
      {selectedNodes.length === 1 && (
        <div>
          <h2>{selectedNodes[0].data.label}</h2>
          <ComponentForm schema={selectedNodes[0].data.inputForm} />
        </div>
      )}
    </div>
  )
}

export default function FlowEditor(
  props: Parameters<typeof FlowEditorInner>[0],
) {
  return (
    <ReactFlowProvider>
      <FlowEditorInner {...props} />
    </ReactFlowProvider>
  )
}
