DualPane = {
	_id = "dual_pane",
}

function DualPane:new(area)
	local me = setmetatable({ _area = area }, { __index = self })
	me:layout()
	me:build()
	return me
end

function DualPane:layout()
	self._chunks = ui.Layout()
		:direction(ui.Layout.HORIZONTAL)
		:constraints({
			ui.Constraint.Ratio(1, 2),
			ui.Constraint.Length(1),
			ui.Constraint.Ratio(1, 2),
		})
		:split(self._area)
end

function DualPane:build()
	local c = self._chunks
	local ratio = { parent = 1, current = 2, preview = 0, all = 3 }
	self._base = {
		ui.Bar(ui.Edge.LEFT)
			:area(c[2])
			:symbol("│")
			:style(ui.Style():patch(th.mgr.border_style)),
	}
	self._children = {
		Tab:new(c[1], cx.tabs[1], cx.tabs.idx == 1, ratio),
		Tab:new(c[3], cx.tabs[2], cx.tabs.idx == 2, ratio),
	}
end

function DualPane:reflow()
	local components = { self }
	for _, child in ipairs(self._children) do
		components = ya.list_merge(components, child:reflow())
	end
	return components
end

function DualPane:redraw()
	local elements = self._base or {}
	for _, child in ipairs(self._children) do
		elements = ya.list_merge(elements, ui.redraw(child))
	end
	return elements
end

function DualPane:click(event, up) end

function DualPane:scroll(event, step) end

function DualPane:touch(event, step) end
